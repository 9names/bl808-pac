#[doc = "Register `cpu_core_cfg7` reader"]
pub struct R(crate::R<CPU_CORE_CFG7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_CORE_CFG7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_CORE_CFG7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_CORE_CFG7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cpu_core_cfg7` writer"]
pub struct W(crate::W<CPU_CORE_CFG7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_CORE_CFG7_SPEC>;
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
impl From<crate::W<CPU_CORE_CFG7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_CORE_CFG7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_pico_div` reader - "]
pub type REG_PICO_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_pico_div` writer - "]
pub type REG_PICO_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_CORE_CFG7_SPEC, u8, u8, 8, O>;
#[doc = "Field `reserved_8_27` reader - "]
pub type RESERVED_8_27_R = crate::FieldReader<u32, u32>;
#[doc = "Field `e902_lpmd_b` reader - "]
pub type E902_LPMD_B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_30` reader - "]
pub type RESERVED_30_R = crate::BitReader<bool>;
#[doc = "Field `pico_rst_mask` reader - "]
pub type PICO_RST_MASK_R = crate::BitReader<bool>;
#[doc = "Field `pico_rst_mask` writer - "]
pub type PICO_RST_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_CORE_CFG7_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_pico_div(&self) -> REG_PICO_DIV_R {
        REG_PICO_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:27"]
    #[inline(always)]
    pub fn reserved_8_27(&self) -> RESERVED_8_27_R {
        RESERVED_8_27_R::new((self.bits >> 8) & 0x000f_ffff)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn e902_lpmd_b(&self) -> E902_LPMD_B_R {
        E902_LPMD_B_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn reserved_30(&self) -> RESERVED_30_R {
        RESERVED_30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pico_rst_mask(&self) -> PICO_RST_MASK_R {
        PICO_RST_MASK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pico_div(&mut self) -> REG_PICO_DIV_W<0> {
        REG_PICO_DIV_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn pico_rst_mask(&mut self) -> PICO_RST_MASK_W<31> {
        PICO_RST_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cpu_core_cfg7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_core_cfg7](index.html) module"]
pub struct CPU_CORE_CFG7_SPEC;
impl crate::RegisterSpec for CPU_CORE_CFG7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_core_cfg7::R](R) reader structure"]
impl crate::Readable for CPU_CORE_CFG7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_core_cfg7::W](W) writer structure"]
impl crate::Writable for CPU_CORE_CFG7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu_core_cfg7 to value 0x01"]
impl crate::Resettable for CPU_CORE_CFG7_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
