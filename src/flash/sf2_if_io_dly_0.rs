#[doc = "Register `sf2_if_io_dly_0` reader"]
pub struct R(crate::R<SF2_IF_IO_DLY_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF2_IF_IO_DLY_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF2_IF_IO_DLY_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF2_IF_IO_DLY_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf2_if_io_dly_0` writer"]
pub struct W(crate::W<SF2_IF_IO_DLY_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF2_IF_IO_DLY_0_SPEC>;
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
impl From<crate::W<SF2_IF_IO_DLY_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF2_IF_IO_DLY_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf2_cs_dly_sel` reader - "]
pub type SF2_CS_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf2_cs_dly_sel` writer - "]
pub type SF2_CS_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF2_IF_IO_DLY_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `sf2_cs2_dly_sel` reader - "]
pub type SF2_CS2_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf2_cs2_dly_sel` writer - "]
pub type SF2_CS2_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF2_IF_IO_DLY_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_4_7` reader - "]
pub type RESERVED_4_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf2_clk_out_dly_sel` reader - "]
pub type SF2_CLK_OUT_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf2_clk_out_dly_sel` writer - "]
pub type SF2_CLK_OUT_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF2_IF_IO_DLY_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_10_25` reader - "]
pub type RESERVED_10_25_R = crate::FieldReader<u16, u16>;
#[doc = "Field `sf2_dqs_oe_dly_sel` reader - "]
pub type SF2_DQS_OE_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf2_dqs_oe_dly_sel` writer - "]
pub type SF2_DQS_OE_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF2_IF_IO_DLY_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `sf2_dqs_di_dly_sel` reader - "]
pub type SF2_DQS_DI_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf2_dqs_di_dly_sel` writer - "]
pub type SF2_DQS_DI_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF2_IF_IO_DLY_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `sf2_dqs_do_dly_sel` reader - "]
pub type SF2_DQS_DO_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf2_dqs_do_dly_sel` writer - "]
pub type SF2_DQS_DO_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF2_IF_IO_DLY_0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf2_cs_dly_sel(&self) -> SF2_CS_DLY_SEL_R {
        SF2_CS_DLY_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sf2_cs2_dly_sel(&self) -> SF2_CS2_DLY_SEL_R {
        SF2_CS2_DLY_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reserved_4_7(&self) -> RESERVED_4_7_R {
        RESERVED_4_7_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf2_clk_out_dly_sel(&self) -> SF2_CLK_OUT_DLY_SEL_R {
        SF2_CLK_OUT_DLY_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:25"]
    #[inline(always)]
    pub fn reserved_10_25(&self) -> RESERVED_10_25_R {
        RESERVED_10_25_R::new(((self.bits >> 10) & 0xffff) as u16)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sf2_dqs_oe_dly_sel(&self) -> SF2_DQS_OE_DLY_SEL_R {
        SF2_DQS_OE_DLY_SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sf2_dqs_di_dly_sel(&self) -> SF2_DQS_DI_DLY_SEL_R {
        SF2_DQS_DI_DLY_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sf2_dqs_do_dly_sel(&self) -> SF2_DQS_DO_DLY_SEL_R {
        SF2_DQS_DO_DLY_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn sf2_cs_dly_sel(&mut self) -> SF2_CS_DLY_SEL_W<0> {
        SF2_CS_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn sf2_cs2_dly_sel(&mut self) -> SF2_CS2_DLY_SEL_W<2> {
        SF2_CS2_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn sf2_clk_out_dly_sel(&mut self) -> SF2_CLK_OUT_DLY_SEL_W<8> {
        SF2_CLK_OUT_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    #[must_use]
    pub fn sf2_dqs_oe_dly_sel(&mut self) -> SF2_DQS_OE_DLY_SEL_W<26> {
        SF2_DQS_OE_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn sf2_dqs_di_dly_sel(&mut self) -> SF2_DQS_DI_DLY_SEL_W<28> {
        SF2_DQS_DI_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn sf2_dqs_do_dly_sel(&mut self) -> SF2_DQS_DO_DLY_SEL_W<30> {
        SF2_DQS_DO_DLY_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf2_if_io_dly_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf2_if_io_dly_0](index.html) module"]
pub struct SF2_IF_IO_DLY_0_SPEC;
impl crate::RegisterSpec for SF2_IF_IO_DLY_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf2_if_io_dly_0::R](R) reader structure"]
impl crate::Readable for SF2_IF_IO_DLY_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf2_if_io_dly_0::W](W) writer structure"]
impl crate::Writable for SF2_IF_IO_DLY_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sf2_if_io_dly_0 to value 0"]
impl crate::Resettable for SF2_IF_IO_DLY_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
