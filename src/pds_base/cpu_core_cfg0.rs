#[doc = "Register `cpu_core_cfg0` reader"]
pub struct R(crate::R<CPU_CORE_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_CORE_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_CORE_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_CORE_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cpu_core_cfg0` writer"]
pub struct W(crate::W<CPU_CORE_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_CORE_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CPU_CORE_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_CORE_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_27` reader - "]
pub type RESERVED_0_27_R = crate::FieldReader<u32, u32>;
#[doc = "Field `reg_pico_clk_en` reader - "]
pub type REG_PICO_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_pico_clk_en` writer - "]
pub type REG_PICO_CLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_CORE_CFG0_SPEC, bool, O>;
#[doc = "Field `e902_dfs_req` reader - "]
pub type E902_DFS_REQ_R = crate::BitReader<bool>;
#[doc = "Field `e902_dfs_req` writer - "]
pub type E902_DFS_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_CORE_CFG0_SPEC, bool, O>;
#[doc = "Field `e902_dfs_ack` reader - "]
pub type E902_DFS_ACK_R = crate::BitReader<bool>;
#[doc = "Field `reserved_31` reader - "]
pub type RESERVED_31_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn reserved_0_27(&self) -> RESERVED_0_27_R {
        RESERVED_0_27_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn reg_pico_clk_en(&self) -> REG_PICO_CLK_EN_R {
        REG_PICO_CLK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn e902_dfs_req(&self) -> E902_DFS_REQ_R {
        E902_DFS_REQ_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn e902_dfs_ack(&self) -> E902_DFS_ACK_R {
        E902_DFS_ACK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reserved_31(&self) -> RESERVED_31_R {
        RESERVED_31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pico_clk_en(&mut self) -> REG_PICO_CLK_EN_W<28> {
        REG_PICO_CLK_EN_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn e902_dfs_req(&mut self) -> E902_DFS_REQ_W<29> {
        E902_DFS_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cpu_core_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_core_cfg0](index.html) module"]
pub struct CPU_CORE_CFG0_SPEC;
impl crate::RegisterSpec for CPU_CORE_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_core_cfg0::R](R) reader structure"]
impl crate::Readable for CPU_CORE_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_core_cfg0::W](W) writer structure"]
impl crate::Writable for CPU_CORE_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu_core_cfg0 to value 0"]
impl crate::Resettable for CPU_CORE_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
